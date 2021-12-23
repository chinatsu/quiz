import { useCallback, useState } from 'react';
import { WebSocket } from 'ws';


export const Websocket = () => {
  const browser = typeof window !== "undefined";
  const [wsInstance, setWsInstance] = useState<null | WebSocket>(null);

// Call when updating the ws connection
  const updateWs = (url: string) => {
    if(!browser && wsInstance !== null) {
      setWsInstance(null);
      return
    }
    
    // Close the old connection
    if(wsInstance?.readyState !== 3)
      wsInstance?.close();

    // Create a new connection
    const newWs = new WebSocket(url);
    setWsInstance(newWs);
  };

  return {wsInstance, updateWs};
}