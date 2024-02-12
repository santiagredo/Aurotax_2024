import { Fragment } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import TestPage from "./test";
import Test2 from "./test2";
import AdminPanel from "./pages/Admin";
import Home from "./pages/Home";
import Header from "./components/Header";
import { PageContextProvider } from "./components/Context";
import { admin, home } from "./utils/routes";

export default function App() {
    return (
        <BrowserRouter>
            <PageContextProvider>
                <Fragment>
                    <Header />

                    <Routes>
                        <Route path={home} element={<Home />} />
                        <Route path="/test" element={<TestPage />} />
                        <Route path="/test2" element={<Test2 />} />
                        <Route path={admin} element={<AdminPanel />} />
                    </Routes>
                </Fragment>
            </PageContextProvider>
        </BrowserRouter>
    );
}
