import './App.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPlus, faSpinner } from '@fortawesome/free-solid-svg-icons';  // Import the "Add" icon
import { faRocket } from '@fortawesome/free-solid-svg-icons';
import { faCheck } from '@fortawesome/free-solid-svg-icons';
import { faTimes } from '@fortawesome/free-solid-svg-icons';
import { faChevronUp, faChevronDown } from '@fortawesome/free-solid-svg-icons';
import { useState, useEffect } from 'react';

import useWebSocket from 'react-use-websocket';

const ws_url = "ws://".concat(window.location.host, "/ws")

function App() {
  return (
    <div className="App">
      <HeadSection />
      <DashboardSection />
      <AddSection />
    </div>
  );
}

export function HeadSection()
{
  return (
  <>
  <div className="header-section">
    <div className="container">
      <div className='logo'>
        <FontAwesomeIcon icon={faRocket} size="2x" />
        <h2>Testnet</h2>
      </div>
    </div>
  </div>
  </>
  );
}

export function DashboardSection()
{
  return (
  <>
    <div className="dashboard-section">
      <div className="container">
        <div className='dashboard'>
          Dashboard
        </div>
      </div>
    </div>
  </>
);
}

export function AddSection()
{
  const [projects, setProjects] = useState({});
  const [stageMap, setStages] = useState({});
  const [isOpenedModal, setIsOpenedModal] = useState(false);
  const [isOpenedLogs, setIsOpenedLogs] = useState([]);

  const { lastMessage } = useWebSocket(ws_url, {
    share: true
  });

  useEffect(() => {
    fetch("/api/projects")
      .then((response) => response.json())
      .then((json) => {
          setProjects({ ...json });

         //setIsOpenedLogs(Array(json.projects.length).fill(false));
      })
      .catch((e) => console.log(e));
  }, []);

  useEffect(() => {
    if (!(lastMessage instanceof Object)) { return }

    const message = JSON.parse(lastMessage.data);
    if (message.notification) {

      return;
    }
    const { project, stage, log } = message;

    // probably the most in efficient way to do it, but hey
    // who cares :P
    if (stageMap[project] && stage in stageMap[project]) {
      setStages({
        ...stageMap,
        [project]: {
          ...stageMap[project],
          [stage]: stageMap[project][stage].concat(log)
        }
      });
    } else {
      // We can merge these two states, no need to have them seperate
      setProjects({...projects,
        [project]: {
          ...projects[project],
          status: "running"
        }
      })
      setStages({
        ...stageMap,
        [project]: {
          ...stageMap[project],
          [stage]: log
        }
      });
    }
  }, [lastMessage])

  function handleLogs(index)
  {
    let next_isOpenedLogs = [...isOpenedLogs];
    next_isOpenedLogs[index] = !next_isOpenedLogs[index];
    setIsOpenedLogs(next_isOpenedLogs);
  }

  function statusIcon(status) {
    if (status === "running") {
      return <FontAwesomeIcon icon={faSpinner} style={{ color: 'orange' }} />
    } else if (status === "finished") {
      return <FontAwesomeIcon icon={faCheck} style={{ color: 'green' }} />
    }
  }

  return (
  <>
  <div className='add-section'>
    <div className='container'>
      <Modal isOpen={isOpenedModal} close={() =>{setIsOpenedModal(false)}}/>
      <div className='projects'>
        {Object.keys(projects).map((key, index) => (
          <div key={key} className='item'>
            <div className='info'>
              <div className='data'>
                <p>{projects[key].name}</p>
                {/* TODO! */}
                <p>c23dsa</p>
              </div>
              <div className='logo'>
                <FontAwesomeIcon icon={faRocket} size="1x" />
              </div>
            </div>
            <div className='result'>
              <button className='show-logs' onClick={() =>{handleLogs(index);}}>
                {isOpenedLogs[index]? (<FontAwesomeIcon icon={faChevronUp} />) : (<FontAwesomeIcon icon={faChevronDown} />)}  
              </button>
              {statusIcon(projects[project].status)}
            </div>
            {isOpenedLogs[index] && <Logs stageMap={stageMap} project={key}/>}
          </div>
        ))}
      </div>
      <div className='add'>
        <div className='add-items'>
          <button onClick={() =>{setIsOpenedModal(true)}}>
            <FontAwesomeIcon icon={faPlus} />
          </button>
          <div>add projects</div>
        </div>
        <div className='add-items'>
          <button>
            <FontAwesomeIcon icon={faPlus} />
          </button>
          <div>add nodes</div>
        </div>
      </div>
    </div>
  </div>
  </>
);
}

export function Modal({isOpen , close})
{
  if(!isOpen)
  {
    return null;
  }
  return (
  <>
  <div className='modal-overlay'>
    <div className='modal'>
      <button className='close-modal' onClick={close}>
        <FontAwesomeIcon icon={faTimes} />
      </button>
      <p className='text'>
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Necessitatibus ipsam suscipit recusandae adipisci fugit laborum repudiandae, temporibus illo eligendi iure officia quibusdam laudantium at ratione libero est corrupti veniam ab.
      </p>
    </div>
  </div>
  </>);
}
export function Logs({ stageMap, project })
{
  const logs = [];

  for (const key in stageMap[project]) {
    logs.push(
    <div>
      <h3>{key}</h3>
      <p style={{whiteSpace: "pre-wrap"}}>{stageMap[project][key]}</p>
    </div>);
  }
  return (
  <>
    <div>
      {logs}
    </div>
  </>);
}



export function FooterSection()
{
  return (
  <>
    <div className="footer-section">
      <div className="container">
        <FontAwesomeIcon icon={faRocket} size="2x" />
        <p class="copyright">&copy; 2024<span>Test net</span> All Right Reserved</p>       
      </div>
    </div>
  </>
);
}

export default App;

