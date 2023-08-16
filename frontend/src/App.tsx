import { FormSelect, Container } from 'react-bootstrap';
import './App.css';

function App() {

  
  return (
    <Container className="App"  data-bs-theme="dark">
        <FormSelect>
          <option value="1">1</option>
          <option value="2">2</option>
          <option value="3">3</option>
        </FormSelect>
    </Container>
  );
}

export default App;
