import {Button, Heading, Ingress, Select, TextField } from '@navikt/ds-react';
import React, { useEffect, useState } from 'react';
import "@navikt/ds-css";
import styles from '../styles/Quiz.module.css';
import router from 'next/router';

const QuizIndex = () => {
  const [quizes, setQuizes] = useState<any[]>([]);
  const [sessions, setSessions] = useState<any[]>([]);
  
  const playSession = async event => {
    event.preventDefault()
    router.push(`/session/${event.target.session.value}`)
  }

  const newSession = async event => {
    event.preventDefault()
    router.push(`/session/new/${event.target.quiz.value}`)
  }

  useEffect(() => {
    fetch(`http://192.168.188.66:3001/quizes`)
      .then(res => res.json())
      .then(data => setQuizes(data))
    fetch(`http://192.168.188.66:3001/sessions`)
      .then(res => res.json())
      .then(data => setSessions(data))
    
  }, [])

  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <Heading size="2xlarge">Quiz app</Heading>
        <Ingress spacing>Here may be quizes</Ingress>
      </header>
      { sessions.length > 0 &&
        <section>
          <form onSubmit={playSession}>
            <Heading size="medium">Join an open session</Heading>
            <Select label="Session ID" name="session">
              {sessions.map(s => <option key={`session-{s.session_id}`} value={s.session_id}>{s.session_id}</option>)}
            </Select>
            <Button className={styles.play} type="submit">Play</Button>
          </form>
          <hr />
        </section>
      }
      <section>
        <Heading size="medium">Create a new session</Heading>
        <form onSubmit={newSession}>
          <Select label="Quiz" name="quiz">
            {quizes.map(q => <option key={`quiz-{q.quiz_id}`} value={q.quiz_id}>{q.name}</option>)}
          </Select>
          <Button className={styles.play} type="submit">New session</Button>
        </form>
      </section>
    </div>
  );
};

export default QuizIndex;