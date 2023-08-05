import React, { useEffect } from 'react'

import { useNavigate } from "react-router-dom";
import Home from '../pages/home'


const PrivateRoute = () => {
    const isLogged = localStorage.getItem('app-token')
    const navigate = useNavigate();
    useEffect(()=>{
        if(isLogged && isLogged !== 'true'){
            navigate("/login")
        }
    },[])
    return isLogged ? <Home /> : <></>
}

export default PrivateRoute