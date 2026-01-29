import { atom } from 'jotai/vanilla';

/**
 * Used to signal the UI that a new “reframe” is available to review.
 * Components can set this when they write a new lesson / reconstruction.
 */
export const reframingAvailableAtom = atom<boolean>(false);
