import { BrowserRouter, Routes, Route } from "react-router-dom";
import './App.css'
import HomePage from './pages/HomePage';

function App() {

  return (
    <>
      <BrowserRouter>
        <Routes>
            <Route index element={<HomePage />} />
        </Routes>
      </BrowserRouter>
    </>
  )
}

export default App
