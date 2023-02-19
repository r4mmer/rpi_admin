import { useState, useEffect } from "react";
import digitalTimerApi from "../api/digitalTimer";
import Table from "react-bootstrap/Table";
import useInterval from "../hooks/useInterval";
import DigitalTimerInput from "./digitalTimerInput";

function DigitalTimerTable() {
  const [timers, setTimers] = useState([]);

  const refreshTimers = () => {
    digitalTimerApi.list().then((res) => {
      setTimers(res);
    });
  };

  const deleteTimer = (id) => {
    digitalTimerApi.delete(id).then((res) => {
      refreshTimers();
    });
  };

  useInterval(() => {
    refreshTimers();
  }, 10000);

  useEffect(() => {
    refreshTimers();
  }, []);

  const onCreate = async (data) => {
    await digitalTimerApi.create(data);
    refreshTimers();
  };

  return (
    <>  
    <Table striped bordered hover variant="dark">
      <thead>
        <tr>
          <th>#</th>
          <th>Enabled</th>
          <th>Name</th>
          <th>Pin</th>
          <th>Time</th>
          <th>Duration</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {timers.map((timer) => (
          <tr key={timer.id}>
            <td>{timer.id}</td>
            <td>{timer.is_enabled ? "Yes" : "No"}</td>
            <td>{timer.name}</td>
            <td>{timer.pin}</td>
            <td>{String(timer.hour).padStart(2, '0')}:{String(timer.minute).padStart(2, '0')}</td>
            <td>{timer.duration}</td>
            <td>
              <button onClick={() => deleteTimer(timer.id)}>Delete</button>
            </td>
          </tr>
        ))}
        <tr>
          <td colSpan={7}>
            <DigitalTimerInput onSubmit={onCreate} />
          </td>
        </tr>
      </tbody>
    </Table>
    </>
  );
}

export default DigitalTimerTable;