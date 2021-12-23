import { Alert, Button, Heading, Ingress } from '@navikt/ds-react';
import React, { useState, useEffect } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import "@navikt/ds-css";
import styles from '../styles/Quiz.module.css';

const Quiz = () => {
  const [socketUrl] = useState('ws://192.168.188.66:3001/quiz/0');
  const [title, setTitle] = useState<string>("");
  const [description, setDescription] = useState<string>("");
  const [currentQuestion, setCurrentQuestion] = useState<any>(null);
  const [ended, setEnded] = useState(false);
  const [score, setScore] = useState<number>(0);
  const [correctAnswers, setCorrectAnswers] = useState<number[]>([]);

  const {
    sendMessage,
    lastMessage,
    readyState,
  } = useWebSocket(socketUrl);


  useEffect(() => {
    console.log(lastMessage?.data);
    if (lastMessage !== null) {
      if (lastMessage.data === "This is supposed to never happen") {
        return;
      }
      let object = JSON.parse(lastMessage.data);
      if (object.message_type === "Quiz") {
          setTitle(object.name);
          setDescription(object.description);
          setEnded(false);
      } else if (object.message_type === "Question") {
        setCorrectAnswers([]);
        setCurrentQuestion(object);
      } else if (object.message_type === "Result") {
        setScore(object.score);
        setCorrectAnswers(object.correct_answers);
      } else if (object.message_type === "End") {
        setCurrentQuestion(null);
        setEnded(true);
      }
    }
  }, [lastMessage]);

  const handleAnswer = (index: number, answer: number) => {
    sendMessage(JSON.stringify({index, answer}));
  }


  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <Heading size="2xlarge">{title}</Heading>
        <Ingress spacing>{description !== "" && description}</Ingress>
        { ended ? <Alert variant="success">Final score: <b>{score}</b></Alert> : <Alert variant="info">Current score: <b>{score}</b></Alert>}
      </header>
      { currentQuestion !== null &&
      <section>
        <Heading spacing size="large">{currentQuestion.text}</Heading>
        {currentQuestion.image_url !== null &&
        <img src={currentQuestion.image_url} alt={currentQuestion.text} />
        }
        <div className={styles.buttonRow}>
        {currentQuestion.alternatives
          .map(a => 
            <Button key={`alternative-${currentQuestion.index}-${a.index}`} className={correctAnswers.length > 0 ? (correctAnswers.includes(a.index) ? styles.correct : styles.wrong) : ""} onClick={e => handleAnswer(currentQuestion.index, a.index)}>{a.text}</Button>
          )
        }
        </div>
      </section>
      }
    </div>
  );
};

export default Quiz;