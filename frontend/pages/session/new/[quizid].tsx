import React from 'react';
import { useRouter } from 'next/router';
import Quiz from '../../../components/Quiz';

const NewSession = () => {
  const router = useRouter();
  const { quizid } = router.query;
  const ws_url = `ws://192.168.188.66:3001/session/new/${quizid}`;
  return <Quiz ws_url={ws_url} />
};

export default NewSession;