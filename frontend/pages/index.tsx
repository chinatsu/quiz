import {Button, Heading, Ingress, TextField } from '@navikt/ds-react';
import React from 'react';
import "@navikt/ds-css";
import styles from '../styles/Quiz.module.css';
import router from 'next/router';

const QuizIndex = () => {
  const playSession = async event => {
    event.preventDefault()
    router.push(`/session/${event.target.session.value}/${event.target.name.value}`)
  }
  return (
    <div className={styles.container}>
      
      <header className={styles.header}>
        <Heading size="2xlarge">Quiz app</Heading>
        <Ingress spacing>Here may be quizes</Ingress>
      </header>
      <section>
      <form onSubmit={playSession}>
        <TextField label="Name" id="name" name="name" required />
        <TextField label="Session" id="session" name="session" type="number" required />
        <Button className={styles.play} type="submit">Play</Button>
      </form>
      </section>
    </div>
  );
};

export default QuizIndex;