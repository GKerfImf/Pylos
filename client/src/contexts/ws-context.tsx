import React, { createContext, useEffect, useRef, useState } from "react";
import useWebSocket from "react-use-websocket";
import { TRequest } from "src/types/request";
import { Response, TResponse } from "src/types/response";
import { toast, useToast } from "src/components/ui/use-toast";

const entryID = "pylos_uuid";
const entryProfileName = "pylos_profile_name";

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

  if (!localStorage.getItem(entryID)) {
    const newPylosUUID = create_UUID();
    localStorage.setItem(entryID, newPylosUUID);
  }

  callback(localStorage.getItem(entryID)!);
};

const generateUserName = (callback: (_: string) => void) => {
  if (!localStorage.getItem(entryID)) {
    console.warn("[ws-context]: [pylos_uuid] does not exist");
  }

  if (!localStorage.getItem(entryProfileName)) {
    callback(`anon-${localStorage.getItem(entryID)!.slice(0, 8)}`);
  } else {
    callback(localStorage.getItem(entryProfileName)!);
  }
};

const registerClient = async (userName: String, userUUID: string, callback: (_: string) => void) => {
  const response = await fetch(`http://localhost:8000/clients/`, {
    method: "POST",
    body: JSON.stringify({ user_name: userName, user_uuid: userUUID }),
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
  const [userName, setUserName] = useState<string>("");
  useEffect(() => {
    generateUserUUID(setUserUUID);
    generateUserName(setUserName);
  }, []);

  // Generated via server
  const [clientUUID, setClientUUID] = useState<string>("");
  useEffect(() => {
    if (!userUUID) {
      return;
    }
    registerClient(userName, userUUID, setClientUUID);
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
    console.debug("[subscribe]", channel, id);
    if (!channels.current.has(channel)) {
      channels.current.set(channel, new Map());
    }
    channels.current.get(channel)?.set(id, callback);
    console.debug("[subscribe]", channels.current.get(channel));
  };

  const unsubscribe = (channel: string, id: string) => {
    console.debug("[unsubscribe]", channel, id);
    if (channels.current.has(channel)) {
      channels.current.get(channel)?.delete(id);
    }
    console.debug("[unsubscribe]", channels.current.get(channel));
  };

  useEffect(() => {
    if (lastMessage != null) {
      const req = Response.parse(JSON.parse(lastMessage.data)) as TResponse;
      const type_req = Object.keys(req)[0];
      if (type_req == undefined) {
        console.log("Error parsing", JSON.parse(lastMessage.data));
      } else {
        toast({
          title: type_req,
          description: lastMessage.data.substring(0, 128) + " ...",
        });
        console.log(type_req);
      }

      if (channels.current.has(type_req)) {
        channels.current.get(type_req)!.forEach((f) => f(req));
      }
    }
  }, [lastMessage]);

  return <WebSocketContext.Provider value={{ subscribe, unsubscribe, send }}>{children}</WebSocketContext.Provider>;
}

export { WebSocketContext, WebSocketProvider };
