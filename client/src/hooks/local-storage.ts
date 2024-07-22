import { useState, useCallback, useEffect } from "react";

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

interface UseLocalStateResult {
  getState: () => string;
  setState: (newState: string) => string;
  setRandom: () => string;
}

function useLocalState(key: string, init?: (uuid: string) => string): UseLocalStateResult {
  const initializeStoredValue = (): void => {
    if (!localStorage.getItem(key)) {
      if (init != null) {
        localStorage.setItem(key, init(create_UUID()));
      } else {
        localStorage.setItem(key, create_UUID());
      }
    }
  };

  const setStoredValue = (value: string): void => {
    if (!localStorage.getItem(key)) {
      throw new Error("[useLocalState, setStoredValue]: trying to change a non-existent local-stored value");
    }
    localStorage.setItem(key, value);
  };

  const setRandomStoredValue = (): void => {
    if (!localStorage.getItem(key)) {
      throw new Error("[useLocalState, setStoredValue]: trying to change a non-existent local-stored value");
    }
    localStorage.setItem(key, create_UUID());
  };

  const getStoredValue = (): string => {
    if (!localStorage.getItem(key)) {
      throw new Error("[useLocalState, getStoredValue]: trying to access a non-existent local-stored value");
    }
    return localStorage.getItem(key)!;
  };

  const [state, setStateInternal] = useState<string>("");

  useEffect(() => {
    initializeStoredValue();
    setStateInternal(getStoredValue());
  });

  const getState = useCallback(() => {
    initializeStoredValue();
    return getStoredValue();
  }, [state]);

  const setState = useCallback((newState: string) => {
    setStoredValue(newState);
    setStateInternal(newState);
    return newState;
  }, []);

  const setRandom = useCallback(() => {
    setRandomStoredValue();
    setStateInternal(getStoredValue());
    return state;
  }, []);

  return { getState, setState, setRandom };
}

export default useLocalState;
