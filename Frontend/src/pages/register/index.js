import React from 'react'

import { ErrorMessage, Formik, Form, Field } from 'formik'
import * as yup from 'yup'
import axios from 'axios'
import { useNavigate } from "react-router-dom";

const Register = () => {
    const navigate = useNavigate();

    const handleSubmit = values => {
        
        const obj = {
            user_name : values.name,
            user_email : values.email,
            user_password: values.password
        }
        axios.post('/createUser', obj)
            .then(resp => {
                const { data } = resp
                if (data) {
                    navigate("/login")
                }
            })
    }

    const validations = yup.object().shape({
        email: yup.string().email().required(),
        password: yup.string().min(8).required()
    })
    return (
        <div className='card'>
            <h1>Register</h1>
            <Formik
                initialValues={{name:'', email:'',password:'' }}
                onSubmit={handleSubmit}
                validationSchema={validations}
            >
                <Form className="Login">
                    <div className="Login-Group">
                        <Field
                            name="name"
                            className="Login-Field"
                            placeholder="User Name"
                        />
                        <ErrorMessage
                            component="span"
                            name="name"
                            className="Login-Error"
                        />
                    </div>
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
                    <button className="Login-Btn" type="submit">Register</button>
                </Form>
            </Formik>
        </div>
    )
}

export default Register
