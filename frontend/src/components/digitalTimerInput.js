import { useState } from 'react';

import Form from 'react-bootstrap/Form';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Button from 'react-bootstrap/Button';

function DigitalTimerInput({onSubmit}) {
    const [validated, setValidated] = useState(false);

    const handleSubmit = (event) => {
        event.preventDefault();
        const form = event.currentTarget;
        if (form.checkValidity() === false) {
            event.stopPropagation();
        }
        setValidated(true);

        const data = {};

        const nameInput = document.getElementById('timer-name-input');
        data.name = nameInput.value;
        const pinInput = document.getElementById('timer-pin-input');
        data.pin = +pinInput.value;
        const timeInput = document.getElementById('timer-time-input');
        data.hour = (+timeInput.value.split(':')[0]) % 24;
        data.minute = +timeInput.value.split(':')[1];
        const durationInput = document.getElementById('timer-duration-input');
        data.duration = +durationInput.value;
        const enabledInput = document.getElementById('timer-enabled-input');
        data.is_enabled = enabledInput.checked;

        if (onSubmit) {
            onSubmit(data).then(() => {
                setValidated(false);
                nameInput.value = '';
                pinInput.value = '';
                timeInput.value = '';
                durationInput.value = '';
                enabledInput.checked = true;
            });
        }
    };

    return (
        <Form noValidate validated={validated} onSubmit={handleSubmit}>
            <Row>
                <Col>
                    <Form.Label>Name</Form.Label>
                </Col>
                <Col>
                    <Form.Label>Pin</Form.Label>
                </Col>
                <Col>
                    <Form.Label>Time</Form.Label>
                </Col>
                <Col>
                    <Form.Label>Duration</Form.Label>
                </Col>
                <Col>
                    <Form.Label>Enabled</Form.Label>
                </Col>
                <Col>{/* Extra column to match cols from inputs */}</Col>
            </Row>
            <Row>
                <Col>
                    <Form.Control id="timer-name-input" placeholder="Name" maxLength={20} />
                </Col>
                <Col>
                    <Form.Control id="timer-pin-input" placeholder="Pin" type='number' min={0} max={20} />
                </Col>
                <Col>
                    <Form.Control id="timer-time-input" placeholder="Time" type='time' />
                </Col>
                <Col>
                    <Form.Control id="timer-duration-input" placeholder="Duration" type='number' min={1} max={86000} />
                </Col>
                <Col>
                    <Form.Check id="timer-enabled-input" type="checkbox" defaultChecked />
                </Col>
                <Col>
                    <Button variant="primary" type="submit">Submit</Button>
                </Col>
            </Row>
        </Form>
    )

}

export default DigitalTimerInput;