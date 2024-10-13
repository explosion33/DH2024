import { BrowserRouter, Routes, Route } from "react-router-dom";
import './App.css'
import HomePage from './pages/HomePage'
import Onboarding from "./pages/Onboarding";
import PeopleView from "./pages/PeopleView";


function App() {

  return (
      <>
        <BrowserRouter>
          <Routes>
            <Route index element={<HomePage />} />
            <Route path="/onboarding" element={<Onboarding />} />
            <Route path="/people" element={<PeopleView />} />
          </Routes>
        </BrowserRouter>
      </>
    )
  
}

export default App
