import logo from './logo.svg';
import './App.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPlus } from '@fortawesome/free-solid-svg-icons';  // Import the "Add" icon
import { faRocket } from '@fortawesome/free-solid-svg-icons';
import { faCheck, faTimes } from '@fortawesome/free-solid-svg-icons';


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
        {/* <div className='data'>
          <button className='add'>
            <FontAwesomeIcon icon={faPlus} />
          </button>
          <div className="notes">add notes</div>
        </div> */}
      </div>
    </div>
  </>
);
}

export function AddSection()
{
  return (
  <>
  <div className='add-section'>
    <div className='container'>
      <div className='projects'>
        <div className='item'>
          <div className='info'>
            <div className='data'>
              <p>Project name1</p>
              <p>Commit id1</p>
            </div>
            <div className='logo'>
              <FontAwesomeIcon icon={faRocket} size="1x" />
            </div>
          </div>
          <div className='result'>
            <FontAwesomeIcon icon={faCheck} style={{ color: 'green' }} />
            {/* <FontAwesomeIcon icon={faTimes} style={{ color: 'red' }} /> Wrong */}
          </div>
        </div>
        
        <div className='item'>
          <div className='info'>
            <div className='data'>
              <p>Project name2</p>
              <p>Commit id3</p>
            </div>
            <div className='logo'>
              <FontAwesomeIcon icon={faRocket} size="1x" />
            </div>
          </div>
          <div className='result'>
            <FontAwesomeIcon icon={faCheck} style={{ color: 'green' }} />
            {/* <FontAwesomeIcon icon={faTimes} style={{ color: 'red' }} /> Wrong */}
          </div>
        </div>
        
        <div className='item'>
          <div className='info'>
            <div className='data'>
              <p>Project name3</p>
              <p>Commit id3</p>
            </div>
            <div className='logo'>
              <FontAwesomeIcon icon={faRocket} size="1x" />
            </div>
          </div>
          <div className='result'>
            <FontAwesomeIcon icon={faCheck} style={{ color: 'green' }} />
            {/* <FontAwesomeIcon icon={faTimes} style={{ color: 'red' }} /> Wrong */}
          </div>
        </div>
        
      </div>
      <div className='add'>
        <div className='add-items'>
          <button>
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




{/* <div className="header-section">
    <div className="container">
      <div className='logo'>
        <FontAwesomeIcon icon={faRocket} size="2x" />
        <span>Test net</span>
      </div>
      <div className='data'>
        <button className='add'>
          <FontAwesomeIcon icon={faPlus} />
        </button>
        <div className="notes">add notes</div>
      </div>
    </div>
  </div> */}