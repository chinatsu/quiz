import React from 'react';
import { useRouter } from 'next/router';
import Quiz from '../../components/Quiz';

const JoinQuiz = () => {
  const router = useRouter();
  const { id } = router.query;
  const ws_url = `ws://192.168.188.66:3001/session/${id}`;
  return <Quiz ws_url={ws_url} />  
};

export default JoinQuiz;