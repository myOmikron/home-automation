import { login, test, logout } from "./auth";
import { handleError } from "./error.ts";
import { Configuration, UserManagementApi } from "./generated";

/** Database id i.e. and u32 */
export type ID = number;

/** Hyphen separated uuid */
export type UUID = string;

const configuration = new Configuration({
    basePath: window.location.origin,
});
const userManagement = new UserManagementApi(configuration);

export const Api = {
    auth: {
        login,
        logout,
        test,
    },
    user: {
        me: () => handleError(userManagement.getMe()),
    },
};
