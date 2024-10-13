import { BrowserRouter, Routes, Route } from "react-router-dom";
import './App.css'
import HomePage from './pages/HomePage'
import { useAuth0 } from "@auth0/auth0-react";
import Navigation from "./components/Navigation";


function App() {
  const { user, isAuthenticated, isLoading } = useAuth0();

  console.log({ user, isAuthenticated, isLoading }); // Debugging output

  // if (isLoading) {
  //   return <div>Loading ...</div>;
  // }
  // isAuthenticated && (

  return (
    
      <>
        <Navigation />

        <BrowserRouter>
          <Routes>
            <Route index element={<HomePage />} />
          </Routes>
        </BrowserRouter>
      </>
    )
  
}

export default App
