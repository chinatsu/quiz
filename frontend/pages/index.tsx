import { Alert, Button, Heading, Ingress } from '@navikt/ds-react';
import React from 'react';
import "@navikt/ds-css";
import styles from '../styles/Quiz.module.css';

const QuizIndex = () => {
  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <Heading size="2xlarge">Quiz app</Heading>
        <Ingress spacing>Here may be quizes</Ingress>
      </header>
      <section>
        <p><a href="quiz/0">Quiz 0?</a></p>
        <p><a href="quiz/1">Quiz 1?</a></p>
      </section>
    </div>
  );
};

export default QuizIndex;