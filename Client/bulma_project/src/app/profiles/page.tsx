'use client';
import React, {useState} from 'react';
import { useRouter, useSelectedLayoutSegment } from "next/navigation";
import { useSelector } from 'react-redux';
import { RootState } from '@/lib/store';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCircle } from '@fortawesome/free-solid-svg-icons';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { library } from '@fortawesome/fontawesome-svg-core';

library.add(faCircle);
library.add(faBars);

export default function Home() {
  const router = useRouter();

  const profileState = useSelector((state: RootState) => state.profile);

  const goBackToCourses = () => {
    router.push('/makePages');
  }

  return (
    <div>
     <div className="box ">
        <h4> Institution:</h4>
        <p className="is-size-4">{profileState.college}</p>
        <h4>Student ID: </h4>
        <p className='is-size-4'> {profileState.studentID}</p>
        <h4>First Name:</h4>
        <p className="is-size-4"> {profileState.firstName}</p>
        <h4> Last Name:</h4>
        <p className="is-size-4"> {profileState.lastName}</p>
        <h4>E-mail:</h4>
        <p className="is-size-4"> {profileState.email}</p>
    </div>
    </div>
    );
}