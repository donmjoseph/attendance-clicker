'use client';
import React from 'react';
import { useRouter } from "next/navigation";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCheckCircle } from '@fortawesome/free-solid-svg-icons';
import styles from './CheckedIn.module.css'; // Importing CSS module

export default function CheckedIn() {

    const router = useRouter();

    const handleBackToClasses = () => {
        router.push('/makePages');
    }

    return (
        <div className={styles.checkedInBox}>
            <div className={styles.header}>
                <h2>CHEM 105 Section 02</h2>
            </div>
            <div className={styles.content}>
                <FontAwesomeIcon icon={faCheckCircle} size="4x" className={styles.icon} />
                <h3 className={styles.title}>You’re checked in!</h3>
                <p className={styles.message}>
                    Stay on this screen to remain in class. We’ll let you know when your instructor starts an activity.
                </p>
            </div>
            <button className={styles.backButton} onClick={handleBackToClasses}>
                Back to Classes
            </button>
        </div>
    );
}
