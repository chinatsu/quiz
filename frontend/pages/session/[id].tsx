import React from 'react';
import { useRouter } from 'next/router';
import Quiz from '../../components/Quiz';

const JoinQuiz = () => {
  const router = useRouter();
  const { id } = router.query;
  const ws_url = `ws://localhost:3001/session/${id}`;
  return <Quiz ws_url={ws_url} />  
};

export default JoinQuiz;