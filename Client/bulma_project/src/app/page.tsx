'use client'
import React, { useState } from 'react';
import './LoginPage.module.css';
import { useRouter } from "next/navigation";
import { Provider } from 'react-redux';
import { useDispatch } from 'react-redux';
import { setEmail, setFirstName, setLastName, setStudentId } from '@/lib/features/profile/profileSlice';
import { AppDispatch } from '@/lib/store';
import { Store } from '@reduxjs/toolkit';
import { makeStore } from '@/lib/store';

interface LoginFormValues {
  email: string;
  password: string;
}

const LoginPage: React.FC = () => {

  const router = useRouter();

  const dispatch = useDispatch<AppDispatch>();

  const goToCourses= () => {

    dispatch(setFirstName("Don"))
    dispatch(setLastName("Jose"))
    dispatch(setEmail("don.jose@wsu.edu"))
    dispatch(setStudentId(1827572))
    router.push('/makePages');
  }  
  
  const [formValues, setFormValues] = useState<LoginFormValues>({
    email: '',
    password: '',
  });

  const [errors, setErrors] = useState<LoginFormValues>({
    email: '',
    password: '',
  });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormValues({
      ...formValues,
      [name]: value,
    });
  };

  const validate = (): boolean => {
    let tempErrors: LoginFormValues = { email: '', password: '' };
    let isValid = true;

    if (!formValues.email) {
      tempErrors.email = 'Email is required';
      isValid = false;
    } else if (!/\S+@\S+\.\S+/.test(formValues.email)) {
      tempErrors.email = 'Email is invalid';
      isValid = false;
    }

    if (!formValues.password) {
      tempErrors.password = 'Password is required';
      isValid = false;
    } else if (formValues.password.length < 6) {
      tempErrors.password = 'Password must be at least 6 characters';
      isValid = false;
    }

    setErrors(tempErrors);
    return isValid;
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (validate()) {
      console.log('Form data:', formValues);
      // Handle successful login logic (e.g., API call)
    }
  };

  return (
    <div className="login-container">
      <h2>Login</h2>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="email">Email</label>
          <input
            type="email"
            id="email"
            name="email"
            value={formValues.email}
            onChange={handleInputChange}
            className={errors.email ? 'error' : ''}
          />
          {errors.email && <div className="error-message">{errors.email}</div>}
        </div>

        <div className="form-group">
          <label htmlFor="password">Password</label>
          <input
            type="password"
            id="password"
            name="password"
            value={formValues.password}
            onChange={handleInputChange}
            className={errors.password ? 'error' : ''}
          />
          {errors.password && <div className="error-message">{errors.password}</div>}
        </div>

        <button onClick={goToCourses} type="submit" className="login-btn">
          Login
        </button>
      </form>
    </div>
  );
};

export default LoginPage;