/* tslint:disable */
/* eslint-disable */
/**
* Requires the input to be manually JSON-serialized to match the `get_config::GetConfigParams` struct.
* @param {string} params
* @returns {string}
*/
export function getConfigDeserializeInput(params: string): string;
/**
* Requires the input as an object that matches the `get_config::GetConfigParams` struct.
* @param {GetConfigParams} params
* @returns {string}
*/
export function getConfigTyped(params: GetConfigParams): string;
export interface GetConfigParams {
    prismaSchema: string;
    triggerError?: boolean;
    env?: Record<string, string>;
}

export interface GetConfigError {
    errorCode: string | null;
    message: string;
}

