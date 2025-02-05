'use client'
import React, { useState } from 'react';
import { useRouter } from "next/navigation";
import styles from './LoginPage.module.css';

export default function LoginPage() {
  return (
    <div className={styles.page}>
      <div className={styles.pagetop}>Log in to WSUClicker</div>
      <div className={styles.log}>
        <div className={`box ${styles.loginContainer}`}>
          <div className={styles.credentials}>Credentials</div>
          <div>
            <div className={styles.gradeContent}>Username</div>
            <input type="text" placeholder="Username" id="username"></input>
          </div>
          <div>
            <div className={styles.gradeContent}>Password</div>
            <input type="text" placeholder="Password" id="password"></input>
          </div>
          <button className={`button is-light ${styles.answerButton}`}>Log In</button>
        </div>
      </div>
    </div>
  );
}