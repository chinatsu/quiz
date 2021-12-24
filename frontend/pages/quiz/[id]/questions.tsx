import { TextField, Button, Radio, RadioGroup } from "@navikt/ds-react";
import { useRouter } from "next/router";
import { useState } from "react";
import "@navikt/ds-css";
import styles from '../styles/NewQuiz.module.css';


const Questions = () => {
    const router = useRouter();
    const { id } = router.query;
    const [question, setQuestion] = useState<any>(null);
    const [answers, setAnswers] = useState<any[]>([]);
 
    const handleNewQuestion = ev => {
        ev.preventDefault();
        const image_url = ev.target.image_url.value;
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                que_text: ev.target.que_text.value, 
                image_url: image_url === "" ? null : image_url,
                answers: answers
            })
        }
        fetch(`http://192.168.188.66:3001/create/quiz/${id}/question`, requestOptions)
            .then(res => res.json())
            .then(data => setQuestion(data))
    }

    const handleNewAnswer = ev => {
        ev.preventDefault();
        setAnswers(answers.concat({
            ans_text: ev.target.ans_text.value,
            correct: ev.target.correct.value === "true" ? true : false
        }));
    }

    const deleteAnswer = (ev, index: number) => {
        ev.preventDefault();
        setAnswers(answers.filter((_,i) => i !== index))
    }

    return (
        <div className={styles.container}>
            <form onSubmit={handleNewQuestion}>
                <TextField label="Text" id="que_text" name="que_text" required autoComplete="off" />
                <TextField label="Image URL" id="image_url" name="image_url" autoComplete="off" />
                <ul>
                    {answers.map((a,i) => <Button variant="tertiary" onClick={e => deleteAnswer(e,i)}>{a.correct ? <b>{a.ans_text}</b> : a.ans_text}</Button>)}
                </ul>
                <Button disabled={answers.filter(a => a?.correct).length === 0} className={styles.submit} type="submit">Create answer</Button>
            </form>
            <hr />
            <h3>Add answer to above question</h3>
            <form onSubmit={handleNewAnswer}>
                <TextField label="Text" id="ans_text" name="ans_text" autoComplete="off" required />
                <RadioGroup legend="Correct" id="correct" default="false" name="correct" autoComplete="off" required>
                    <Radio value="true">True</Radio>
                    <Radio value="false">False</Radio>
                </RadioGroup>
                <Button className={styles.submit} variant="secondary">Add new answer</Button>
            </form>
            <hr />
            {question === null ? null 
            : <div>{JSON.stringify(question)}</div>
            }
        </div>
    )
}

export default Questions;