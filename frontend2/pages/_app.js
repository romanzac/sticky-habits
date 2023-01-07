import { HabitProvider } from "../context/HabitList";
import "../styles/globals.css";

const MyApp = ({ Component, pageProps }) => (
  <HabitProvider>
    <div>
      <Component {...pageProps} />
    </div>
  </HabitProvider>
);

export default MyApp;
