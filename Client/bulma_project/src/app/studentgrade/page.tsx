import styles from "./page.module.css";
import { useRouter } from 'next/navigation';


export default function Home() {
    return (
      <div className={styles.page}>
        <div className={styles.classname}>Grades for Class Name</div>
          <div className={styles.gradeBox}>
          <div className={`box ${styles.grade}`}>
            <div className={styles.gradeContent}>Course Score</div>
          </div>
          <div className={`box ${styles.grade}`}>
            <div className={styles.gradeContent}>Attendance</div>
          </div>
          </div>
      {/*<button className={`button is-light ${styles.submitButton}`}>Submit</button>*/}
      </div>
    );
  }