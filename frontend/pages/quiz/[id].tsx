import { Alert, Button, Heading, Ingress } from '@navikt/ds-react';
import React, { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';
import "@navikt/ds-css";
import styles from '../../styles/Quiz.module.css';
import { useRouter } from 'next/router';

const Quiz = () => {
  const router = useRouter();
  const { id } = router.query;
  const [title, setTitle] = useState<string>("");
  const [description, setDescription] = useState<string>("");
  const [currentQuestion, setCurrentQuestion] = useState<any>(null);
  const [ended, setEnded] = useState(false);
  const [score, setScore] = useState<number>(0);
  const [total, setTotal] = useState<number>(0);
  const [correctAnswers, setCorrectAnswers] = useState<number[]>([]);
  const [answered, setAnswered] = useState(true);

  const {
    sendMessage,
    lastMessage,
  } = useWebSocket(`ws://192.168.188.66:3001/quiz/${id}`);


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
        setTotal(object.num_questions);
        setEnded(false);
      } else if (object.message_type === "Question") {
        setCorrectAnswers([]);
        setAnswered(false);
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
    setAnswered(true);
    console.log(answered);
    sendMessage(JSON.stringify({index, answer}));
  }


  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <Heading size="2xlarge">{title}</Heading>
        <Ingress spacing>{description !== "" && description}</Ingress>
        { ended ? <Alert variant="success">Final score: <b>{score} out of {total}</b></Alert> : <Alert variant="info">Current score: <b>{score}</b></Alert>}
      </header>
      { currentQuestion !== null &&
      <section>
        <Heading spacing size="large">{currentQuestion.text}</Heading>
        {currentQuestion.image_url !== null &&
        <img className={styles.image} src={currentQuestion.image_url} alt={currentQuestion.text} />
        }
        <div className={styles.buttonRow}>
        {currentQuestion.alternatives
          .map(a => 
            <Button disabled={answered}  key={`alternative-${currentQuestion.index}-${a.index}`} className={
                correctAnswers.length > 0 
                ? (correctAnswers.includes(a.index) ? styles.correct : styles.wrong) 
                : ""} onClick={() => answered ? null : handleAnswer(currentQuestion.index, a.index)}>{a.text}</Button>
          )
        }
        </div>
      </section>
      }
    </div>
  );
};

export default Quiz;