import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "react-hot-toast";
import { BrowserRouter } from "react-router-dom";
import Body from "./Body";
import { DarkModeProvider } from "./components/DarkModeProvider";
import "./i18next";

const queryClient = new QueryClient();
const App = () => {
  return (
    <BrowserRouter>
      <QueryClientProvider client={queryClient}>
        <DarkModeProvider>
          <Body />
          <Toaster position="bottom-center" />
        </DarkModeProvider>
      </QueryClientProvider>
    </BrowserRouter>
  );
};

export default App;
