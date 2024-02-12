import { createContext, useState } from "react";
import { UserModel } from "../utils/models";

interface PageContextValue {
    userDetails: UserModel;
    setUserDetails: React.Dispatch<React.SetStateAction<UserModel>>;

    loginModal: boolean;
    setLoginModalOpen: React.Dispatch<React.SetStateAction<boolean>>;

    logoutModal: boolean;
    setLogoutModalOpen: React.Dispatch<React.SetStateAction<boolean>>;

    errorModal: boolean;
    setErrorModalOpen: React.Dispatch<React.SetStateAction<boolean>>;
}

export const PageContext = createContext<PageContextValue>({
    userDetails: { userEmail: "", userName: "", paypalOrderId: "", userId: 0 },
    setUserDetails: () => {},

    loginModal: false,
    setLoginModalOpen: () => {},

    logoutModal: false,
    setLogoutModalOpen: () => {},

    errorModal: false,
    setErrorModalOpen: () => {},
});

interface PageContextProps {
    children: React.ReactNode;
}

export const PageContextProvider = ({ children }: PageContextProps) => {
    const [userDetails, setUserDetails] = useState<UserModel>({
        userEmail: "",
        userName: "",
        paypalOrderId: "",
        userId: 0
    });

    const [loginModal, setLoginModalOpen] = useState<boolean>(false);

    const [logoutModal, setLogoutModalOpen] = useState<boolean>(false);

    const [errorModal, setErrorModalOpen] = useState<boolean>(false);

    return (
        <PageContext.Provider
            value={{
                userDetails,
                setUserDetails,
                loginModal,
                setLoginModalOpen,
                logoutModal,
                setLogoutModalOpen,
                errorModal,
                setErrorModalOpen,
            }}
        >
            {children}
        </PageContext.Provider>
    );
};
