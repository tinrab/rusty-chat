import { InputType, JoinInput, PostInput } from './types';

function join(name: string): JoinInput {
    return { type: InputType.Join, payload: { name } };
}

function post(body: string): PostInput {
    return { type: InputType.Post, payload: { body } };
}

const apiProto = {
    join,
    post,
};

export default apiProto;
