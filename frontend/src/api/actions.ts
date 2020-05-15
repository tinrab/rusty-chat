import { ApiActionType, Input, Output, ReadApiAction, WriteApiAction } from './types';

function write(input: Input): WriteApiAction {
    return { type: ApiActionType.Write, payload: input };
}

function read(output: Output): ReadApiAction {
    return { type: ApiActionType.Read, payload: output };
}

const apiActions = {
    write,
    read,
};

export default apiActions;
