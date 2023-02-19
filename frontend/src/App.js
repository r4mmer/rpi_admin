import logo from './logo.svg';
import './App.css';
import DigitalTimerTable from './components/digitalTimerTable';

import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';

function App() {
  return (
    <>
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
      </header>
      <Tabs
        defaultActiveKey="digital-timers"
        id="tabbed-tbl-actions"
        className="mb-3"
      >
        <Tab eventKey="digital-timers" title="Digital timers">
          <DigitalTimerTable />
        </Tab>
      </Tabs>
    </div>
    </>
  );
}

export default App;
