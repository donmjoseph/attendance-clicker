'use client'
import Image from "next/image";
import styles from "./page.module.css";
import { useRouter } from "next/navigation";

export default function Home() {

  const router = useRouter();

  const goToClasses = () => {
    router.push('/makePages');
  }

  return (
    <div className={styles.page}>
      <div className={styles.QNA}>
        <div className={styles.leftsection}>
          {/*<button className={styles.button} style={{
            backgroundColor: '#32cd32'
            }}
          >
          
          </button>*/}
          <button className={`button is-light ${styles.answerButton}`}>A</button>
          <button className={`button is-light ${styles.answerButton}`}>B</button>
        </div>
      <div className={styles.rightsection}></div>
      </div>
      <button className={`button is-light ${styles.submitButton}`} onClick={goToClasses}>Submit</button>
      </div>
  );
}
