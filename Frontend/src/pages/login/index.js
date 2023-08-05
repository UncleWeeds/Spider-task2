import React from 'react'

import { ErrorMessage, Formik, Form, Field } from 'formik'
import * as yup from 'yup'
import axios from 'axios'
import { useNavigate } from "react-router-dom";

const Login =  () => {
    const navigate = useNavigate();
    const handleSubmit = values => {
        const obj = {
            user_email : values.email,
            user_password: values.password
        }
        axios.post('/loginUser', obj)
            .then(async (resp) => {
                const { data } = resp
                if (data) {
                    await localStorage.setItem('app-token', true)
                    
                    navigate("/home")
                }else{
                    navigate("/login")
                }
            })
            .catch(err => {
                alert("User not authenticated")
            })
    }

    const validations = yup.object().shape({
        email: yup.string().email().required(),
        password: yup.string().min(8).required()
    })
    return (
        <div className='card'>
            <h1>Login</h1>
            <Formik
                initialValues={{ email:'',password:'' }}
                onSubmit={handleSubmit}
                validationSchema={validations}
                className=""
            >
                <Form className="Login">
                    <div className="Login-Group">
                        <Field
                            name="email"
                            className="Login-Field"
                            placeholder="Email"
                        />
                        <ErrorMessage
                            component="span"
                            name="email"
                            className="Login-Error"
                        />
                    </div>
                    <div className="Login-Group">
                        <Field
                            name="password"
                            className="Login-Field"
                            placeholder="Password"
                            type="password"
                        />
                        <ErrorMessage
                            component="span"
                            name="password"
                            className="Login-Error"
                        />
                    </div>
                    <button className="Login-Btn" type="submit">Login</button>
                </Form>
            </Formik>
        </div>
    )
}

export default Login
