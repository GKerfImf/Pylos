import React, { createContext, useEffect, useRef, useState } from "react";
import useWebSocket, { ReadyState, SendMessage } from "react-use-websocket";
import { TRequest } from "src/types/request";
import { AvailableGames, GameCreated, GameState, TAvailableGames, TGameCreated, TGameState } from "src/types/response";

const generateUserUUID = (callback: (_: string) => void) => {
  // https://hitchhikers.yext.com/guides/analyze-trends-with-visitor-analytics/07-cookies-visitors/
  function create_UUID() {
    var dt = new Date().getTime();
    var uuid = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
      var r = (dt + Math.random() * 16) % 16 | 0;
      dt = Math.floor(dt / 16);
      return (c == "x" ? r : (r & 0x3) | 0x8).toString(16);
    });
    return uuid;
  }

  const entryID = "pylos_uuid";
  if (!localStorage.getItem(entryID)) {
    const newPylosUUID = create_UUID();
    localStorage.setItem(entryID, newPylosUUID);
  }

  callback(localStorage.getItem(entryID)!);
};

const registerClient = async (userUUID: string, callback: (_: string) => void) => {
  const response = await fetch(`http://localhost:8000/clients/`, {
    method: "POST",
    body: JSON.stringify({ user_uuid: userUUID }),
    credentials: "include",
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  }).then((response) => response.json());
  callback(await response.url.split("/").at(-1));
};

const unregisterClient = async (userUUID: string) => {
  const response = await fetch(`http://localhost:8000/clients/${userUUID}`, {
    method: "DELETE",
    credentials: "include",
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  }).then((response) => response.json());
};

type WebSocketContextProps = {
  send: (req: TRequest) => void;
  subscribe: (channel: string, id: string, b: any) => void;
  unsubscribe: (channel: string, id: string) => void;
};

const WebSocketContext = createContext<WebSocketContextProps | null>(null);
function WebSocketProvider({ children }: { children: any }) {
  // Stored in [localStorage]
  const [userUUID, setUserUUID] = useState<string>("");
  useEffect(() => {
    generateUserUUID(setUserUUID);
  }, []);

  // Generated via server
  const [clientUUID, setClientUUID] = useState<string>("");
  useEffect(() => {
    if (!userUUID) {
      return;
    }
    registerClient(userUUID, setClientUUID);
    return () => {
      unregisterClient(clientUUID);
    };
  }, [userUUID]);

  // Setup the socket connection
  const [socketUrl, setSocketUrl] = useState("ws://127.0.0.1:8000/ws/");
  useEffect(() => {
    if (clientUUID == "") {
      return;
    }
    console.debug("[useEffect, setSocketUrl]");
    setSocketUrl(`ws://127.0.0.1:8000/ws/${clientUUID}`);
  }, [userUUID, clientUUID]);

  const { sendMessage, lastMessage } = useWebSocket<string>(socketUrl);

  const send = (req: TRequest) => {
    sendMessage(JSON.stringify(req));
  };

  const channels = useRef<Map<string, Map<string, any>>>(new Map());
  const subscribe = (channel: string, id: string, callback: (a: any) => void) => {
    console.log("[subscribe]", channel, id);
    if (!channels.current.has(channel)) {
      channels.current.set(channel, new Map());
    }
    channels.current.get(channel)?.set(id, callback);
    console.log("[subscribe]", channels.current.get(channel));
  };

  const unsubscribe = (channel: string, id: string) => {
    console.log("[unsubscribe]", channel, id);
    if (channels.current.has(channel)) {
      channels.current.get(channel)?.delete(id);
    }
    console.log("[unsubscribe]", channels.current.get(channel));
  };

  useEffect(() => {
    if (lastMessage != null) {
      const untyped_req = JSON.parse(lastMessage.data);
      switch (Object.keys(untyped_req)[0]) {
        case "AvailableGames": {
          const req = AvailableGames.parse(untyped_req) as TAvailableGames;
          if (channels.current.has("AvailableGames")) {
            channels.current.get("AvailableGames")!.forEach((f) => f(req));
          }
          break;
        }
        case "GameCreated": {
          const req = GameCreated.parse(untyped_req) as TGameCreated;
          if (channels.current.has("GameCreated")) {
            channels.current.get("GameCreated")!.forEach((f) => f(req));
          }
          break;
        }
        case "GameState": {
          const req = GameState.parse(untyped_req) as TGameState;
          if (channels.current.has("GameState")) {
            channels.current.get("GameState")!.forEach((f) => f(req));
          }
          break;
        }
        default: {
          console.warn("Unknown request", lastMessage.data);
        }
      }
    }
  }, [lastMessage]);

  return <WebSocketContext.Provider value={{ subscribe, unsubscribe, send }}>{children}</WebSocketContext.Provider>;
}

export { WebSocketContext, WebSocketProvider };
