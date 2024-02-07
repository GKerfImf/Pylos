import React, { createContext, useEffect, useState } from "react";
import useWebSocket, { ReadyState, SendMessage } from "react-use-websocket";

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

//

type WebSocketContextProps = {
  connectionStatus: String;
  sendJsonMessage: SendMessage;
  lastMessage: MessageEvent<any> | null;
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

  const { sendJsonMessage, lastMessage, readyState } = useWebSocket<string>(socketUrl);
  const connectionStatus = {
    [ReadyState.CONNECTING]: "Connecting",
    [ReadyState.OPEN]: "Open",
    [ReadyState.CLOSING]: "Closing",
    [ReadyState.CLOSED]: "Closed",
    [ReadyState.UNINSTANTIATED]: "Uninstantiated",
  }[readyState];

  return (
    <WebSocketContext.Provider value={{ connectionStatus, sendJsonMessage, lastMessage }}>
      {children}
    </WebSocketContext.Provider>
  );
}

export { WebSocketContext, WebSocketProvider };
