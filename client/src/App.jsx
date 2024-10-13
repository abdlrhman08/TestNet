import './App.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPlus } from '@fortawesome/free-solid-svg-icons';  // Import the "Add" icon
import { faRocket } from '@fortawesome/free-solid-svg-icons';
import { faCheck } from '@fortawesome/free-solid-svg-icons';
import { faTimes } from '@fortawesome/free-solid-svg-icons';
import { faChevronUp, faChevronDown } from '@fortawesome/free-solid-svg-icons';
import { useState, useEffect } from 'react';


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
  const [isOpenedModal, setIsOpenedModal] = useState(false);
  const [isOpenedLogs, setIsOpenedLogs] = useState([]);
  useEffect(() => {
    fetch("/api/projects")
      .then((response) => response.json())
      .then((json) => 
        {setProjects({ ...json });
         setIsOpenedLogs(Array(json.projects.length).fill(false));})
      .catch((e) => console.log(e));
  }, []);

  function handleLogs(index)
  {
    let next_isOpenedLogs = [...isOpenedLogs];
    next_isOpenedLogs[index] = !next_isOpenedLogs[index];
    setIsOpenedLogs(next_isOpenedLogs);
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
              <FontAwesomeIcon icon={faCheck} style={{ color: 'green' }} />
              {/* <FontAwesomeIcon icon={faTimes} style={{ color: 'red' }} /> Wrong */}
            </div>
            {isOpenedLogs[index] && <Logs />}
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
export function Logs()
{
  return (
  <>
  <div className='logs'>
    <p>Log1</p>
    <p>Log2</p>
    <p>Log3</p>
    <p>Log4</p>
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

