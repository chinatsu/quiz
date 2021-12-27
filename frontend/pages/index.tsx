import {Button, Heading, Ingress, Link, TextField } from '@navikt/ds-react';
import React from 'react';
import "@navikt/ds-css";
import styles from '../styles/Quiz.module.css';
import router from 'next/router';

const QuizIndex = () => {
  const playSession = async event => {
    event.preventDefault()
    router.push(`/session/${event.target.session.value}`)
  }

  const newSession = async event => {
    event.preventDefault()
    router.push(`/session/new/${event.target.quiz.value}`)
  }
  return (
    <div className={styles.container}>
      
      <header className={styles.header}>
        <Heading size="2xlarge">Quiz app</Heading>
        <Ingress spacing>Here may be quizes</Ingress>
      </header>
      <section>
      <form onSubmit={playSession}>
        <TextField label="Session ID" id="session" name="session" type="number" required />
        <Button className={styles.play} type="submit">Play</Button>
      </form>
      </section>
      <hr />
      <section>
        <Heading size="medium">Alternatively, create a new session</Heading>
        <form onSubmit={newSession}>
          <TextField label="Quiz ID" id="quiz" name="quiz" type="number" required />
          <Button className={styles.play} type="submit">New session</Button>
        </form>
      </section>
    </div>
  );
};

export default QuizIndex;