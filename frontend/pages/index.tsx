import React, { useState, useEffect, useCallback } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';

const Quiz = () => {
  const [socketUrl, setSocketUrl] = useState('ws://192.168.188.66:3001/quiz/1');
  const [messageHistory, setMessageHistory] = useState<any>([]);

  const {
    sendMessage,
    lastMessage,
    readyState,
  } = useWebSocket(socketUrl);

  useEffect(() => {
    if (lastMessage !== null) {
      setMessageHistory(prev => prev.concat(lastMessage));
    }
  }, [lastMessage, setMessageHistory]);

  const handleClickSendMessage = useCallback(() =>
    sendMessage('1'), []);

  const connectionStatus = {
    [ReadyState.CONNECTING]: 'Connecting',
    [ReadyState.OPEN]: 'Open',
    [ReadyState.CLOSING]: 'Closing',
    [ReadyState.CLOSED]: 'Closed',
    [ReadyState.UNINSTANTIATED]: 'Uninstantiated',
  }[readyState];

  return (
    <div>
      <button
        onClick={handleClickSendMessage}
        disabled={readyState !== ReadyState.OPEN}
      >
        Click Me to send '1'
      </button>
      <span>The WebSocket is currently {connectionStatus}</span>
      <ul>
        {messageHistory
          .map((message, idx) => <p key={idx}>{message ? message.data : null}</p>)}
      </ul>
    </div>
  );
};

export default Quiz;