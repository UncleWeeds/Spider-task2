import React from 'react'

// import Routes from '../components/Routes'

import { Route, Routes,Navigate } from "react-router-dom";

import Login from '../pages/login'
import Register from '../pages/register'
// import Home from '../pages/home'
import NotFound from '../components/NotFound'
import PrivateRoute from '../components/PrivateRoute'

const App = () => (
    <main className="App">
         <Routes>
            
            <Route path="/" element={ <Navigate to="/login" /> } />
            <Route element={<Register />}  path="/register" />
            <Route element={<Login />}  path="/login" />
            <Route element={<PrivateRoute />}  path="/home" />
            <Route element={<NotFound />} />
        </Routes >
    </main>
)

export default App
