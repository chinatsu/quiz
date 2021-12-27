import { Alert, Button, Heading, Ingress, TextField } from '@navikt/ds-react';
import { Star } from '@navikt/ds-icons';
import React, { useState, useEffect } from 'react';
import useWebSocket from 'react-use-websocket';
import "@navikt/ds-css";
import styles from '../styles/Quiz.module.css';


const Quiz = (props) => {
    const { ws_url } = props;
    const [title, setTitle] = useState<string>("");
    const [description, setDescription] = useState<string>("");
    const [currentQuestion, setCurrentQuestion] = useState<any>(null);
    const [currentPlayerResults, setCurrentPlayerResults] = useState<any>(null);
    const [players, setPlayers] = useState<any[]>([]);
    const [ended, setEnded] = useState(false);
    const [nameNeeded, setNameNeeded] = useState(false);
    const [score, setScore] = useState<number>(0);
    const [total, setTotal] = useState<number>(0);
    const [correctAnswers, setCorrectAnswers] = useState<number[]>([]);
    const [answered, setAnswered] = useState(true);
    const [name, setName] = useState("");
    const [sessionId, setSessionId] = useState<number | null>(null);
  
    const {
      sendMessage,
      lastMessage,
    } = useWebSocket(ws_url);
  
  
    useEffect(() => {
      if (lastMessage !== null) {
          if (lastMessage.data === "This is supposed to never happen") {
            return;
          }
          let object = JSON.parse(lastMessage.data);
          switch (object.message_type) {
              case "Quiz": {
                  setTitle(object.name);
                  setDescription(object.description);
                  setTotal(object.num_questions);
                  setEnded(false);
                  setCurrentPlayerResults(null);
                  setPlayers([]);
                  break;
              }
              case "Question": {
                  setCorrectAnswers([]);
                  setAnswered(false);
                  setCurrentQuestion(object);
                  break;    
              }
              case "Result": {
                  setScore(object.score);
                  setCorrectAnswers(object.correct_answers);
                  break;    
              }
              case "End": {
                  setCurrentQuestion(null);
                  setEnded(true);
                  break;        
              }
              case "NameRequest": {
                  setPlayers([]);
                  setNameNeeded(true);
                  break;    
              }
              case "PlayerResults": {
                setCurrentPlayerResults(object);
                break;
              }
              case "PlayerList": {
                  setPlayers(object.players);
                  break;   
              }
              case "SessionInfo": {
                  setSessionId(object.session_id);
                  break;
              }
              default: {
                  console.log("Unimplemented: " + JSON.stringify(object))
              }
          }
      }
    }, [lastMessage]);
  
    const handleAnswer = (index: number, answer: number) => {
      setAnswered(true);
      sendMessage(JSON.stringify({index, answer}));
    }
  
    const handleNameChange = ev => {
      ev.preventDefault();
      const name = ev.target.name.value;
      sendMessage(JSON.stringify({name}));
      setName(name);
      setNameNeeded(false);
    }
  
    const handleStart = () => {
        sendMessage(JSON.stringify({message_type: "Start"}))
    }
  
  
    return (
      <div className={styles.container}>
        <header className={styles.header}>
          <Heading size="2xlarge">{title}</Heading>
          <Ingress spacing>{description !== "" && description}</Ingress>
          <Ingress spacing>{sessionId && `Session ${sessionId}`}</Ingress>
          { currentPlayerResults !== null ? null : (ended ? <Alert variant="success">Final score: <b>{score} out of {total}</b></Alert> : <Alert variant="info">Current score: <b>{score}</b></Alert>)}
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
              <Button disabled={answered} key={`alternative-${currentQuestion.index}-${a.index}`} className={
                  correctAnswers.length > 0 
                  ? (correctAnswers.includes(a.index) ? styles.correct : styles.wrong) 
                  : ""} onClick={() => answered ? null : handleAnswer(currentQuestion.index, a.index)}>{a.text}</Button>
            )
          }
          </div>
        </section>
        }
        { currentPlayerResults !== null &&
        <section>
          <Heading spacing size="large">
          { currentPlayerResults.game_ended 
            ? "Game over"
            : "Game in progress"
          }
          </Heading>
          <ol>
            { [...currentPlayerResults.players].sort((a, b) => b.score - a.score).map(p => 
              <li key={`player-${p.player_id}`}>{p.name == name ? <b>{p.name}</b> : p.name}: <b>{p.score}</b> {p.finished ? null : "(Not finished yet)"}</li>)}
          </ol>
        </section>
        }
        { nameNeeded &&
        <section>
            <Heading spacing size="large">
                Please enter your name
            </Heading>
  
            <form onSubmit={handleNameChange}>
              <TextField label="Name" id="name" name="name" required />
              <Button className={styles.play} type="submit">Submit</Button>
            </form>
        </section>
        }
        { players.length > 0 &&
        <section>
            <Heading spacing size="large">
                Waiting for the session to start
            </Heading>
            <ul>
              {players.map(p => {
                  let leader = p.host ? <Star /> : null;
                  return <li key={`sessionPlayer-${p.player_id}`}>{p.name == name ? <b>{p.name}</b> : p.name} {leader}</li>
              })}
            </ul>
            { players.find(p => p.name == name)?.host &&
              <Button onClick={handleStart}>Start session</Button>
            }
        </section>
        }
      </div>
    );
};

export default Quiz;