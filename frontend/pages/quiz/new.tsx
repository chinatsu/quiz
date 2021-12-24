import { Button, TextField } from '@navikt/ds-react';
import { useState } from 'react';
import "@navikt/ds-css";
import styles from './styles/NewQuiz.module.css';

const NewQuiz = () => {
    const [quiz, setQuiz] = useState<any>(null);

    const handleNewQuiz = ev => {
        ev.preventDefault()
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                name: ev.target.name.value, 
                description: ev.target.description.value
            })
        }
        fetch('http://192.168.188.66:3001/create/quiz', requestOptions)
            .then(res => res.json())
            .then(data => setQuiz(data))
    }
    
    return (
        <div className={styles.container}>
            <form onSubmit={handleNewQuiz}>
                <TextField label="Name" id="name" name="name" autoComplete="off" required />
                <TextField label="Description" id="description" name="description" autoComplete="off" required />
                <Button className={styles.submit} type="submit">Create quiz</Button>
            </form>
            {quiz !== null 
            ? <a href={`/quiz/${quiz.qui_id}/questions`}>Go to administer your quiz</a>
            : null
            }
        </div>
    )
}

export default NewQuiz;