import '../app/globals.css';
import { config } from '@fortawesome/fontawesome-svg-core';
import '@fortawesome/fontawesome-svg-core/styles.css'; 
config.autoAddCss = false; 
import '@/fontawesome';
import { AppProps } from 'next/app';


function MyApp({ Component, pageProps }: AppProps) {



  return <Component {...pageProps} />;
}

export default MyApp;
