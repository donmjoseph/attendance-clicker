'use client';
import React from 'react';
import { useRouter } from "next/navigation";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCircle } from '@fortawesome/free-solid-svg-icons';
import { library } from '@fortawesome/fontawesome-svg-core';

library.add(faCircle);

export default function Home() {

    const router = useRouter();

    const goToClasses = () => {
        router.push('/');
    }

  return (
    <div className="box custom-box">
    <h2> In session</h2>
    <button className='button class-button'>
        <p> <FontAwesomeIcon icon="circle"/> CPTS 101 - Prof John Doe </p>
        <p>MWF 1:00 PM to 3:00 PM</p>
    </button>    

    <div>
        
    </div>
    <button className='button class-button'>
        <p>BIO 102 - Prof Jane Doe </p>
        <p>Tue/Thurs 3:00 PM to 5:00 PM</p>
    </button>    
    <button className='button class-button'>
        <p>PHYS 104 - Prof Jack Doe </p>
        <p>Tue/Thurs 11:00 AM to 9:00 PM</p>
    </button>    
    <button className='button class-button'>
        <p>CHEM 103 - Prof Jame Doe </p>
        <p>MWF 11:00 AM to 12:00 PM</p>
    </button>    
    </div>
    );
}