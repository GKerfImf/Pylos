import { useLocalStorage } from "@uidotdev/usehooks";
import React, { createContext, useEffect, useRef, useState } from "react";
import useWebSocket from "react-use-websocket";
import { TRequest } from "src/types/request";
import { Response, TResponse } from "src/types/response";
import generateDefaultName from "src/util/default-names";
import createUUID from "src/util/uuid";

const url = window.location.host;
// const url = "localhost:8000";

const registerClient = async (
  userName: String,
  userUUID: string,
  userAvatar: string,
  callback: (_: string) => void
) => {
  const response = await fetch(`http://${url}/clients/`, {
    method: "POST",
    body: JSON.stringify({ user_name: userName, user_uuid: userUUID, user_avatar_uuid: userAvatar }),
    credentials: "include",
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  }).then((response) => response.json());
  callback(await response.url.split("/").at(-1));
};

const unregisterClient = async (userUUID: string) => {
  const response = await fetch(`http://${url}/clients/${userUUID}`, {
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
  const [nameLocal] = useLocalStorage<string>("PylosProfileName", generateDefaultName());
  const [userUUID] = useLocalStorage<string>("PylosProfileUUID", createUUID());
  const [avatarLocal] = useLocalStorage<string>("PylosProfileAvatar", createUUID());

  // Generated via server
  const [clientUUID, setClientUUID] = useState<string>("");
  useEffect(() => {
    if (!userUUID) {
      return;
    }
    registerClient(nameLocal, userUUID, avatarLocal, setClientUUID);
    return () => {
      unregisterClient(clientUUID);
    };
  }, [userUUID]);

  // Setup the socket connection
  const [socketUrl, setSocketUrl] = useState(`ws://${url}/ws/`);
  useEffect(() => {
    if (clientUUID == "") {
      return;
    }
    console.debug("[useEffect, setSocketUrl]");
    setSocketUrl(`ws://${url}/ws/${clientUUID}`);
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
        console.error("Error parsing", JSON.parse(lastMessage.data));
      } else {
        console.log(req);
      }

      if (channels.current.has(type_req)) {
        channels.current.get(type_req)!.forEach((f) => f(req));
      }
    }
  }, [lastMessage]);

  return <WebSocketContext.Provider value={{ subscribe, unsubscribe, send }}>{children}</WebSocketContext.Provider>;
}

export { WebSocketContext, WebSocketProvider };
