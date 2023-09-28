import * as anchor from '@coral-xyz/anchor';
import { Program, web3 } from '@coral-xyz/anchor';
import { TicTacToeAnchor } from '../target/types/tic_tac_toe_anchor';
import { generateKeyPair, sendAnchorInstructions, printParticipants } from './utils'

anchor.setProvider(anchor.AnchorProvider.env());
const connection = anchor.AnchorProvider.env().connection;
const program = anchor.workspace.TicTacToeAnchor as Program<TicTacToeAnchor>;

describe('TicTacToeAnchor', async () => {
  let initializer: web3.Keypair;
  let playerA: web3.Keypair;
  let playerB: web3.Keypair;
  const lobbyName = 'test-lobby' + Math.random();
  const delaySlots = 1000;
  const requiredAmountInLamports = 1000000;

  before(async () => {
    [initializer, playerA, playerB] = await Promise.all([
      generateKeyPair(connection, 1),
      generateKeyPair(connection, 1),
      generateKeyPair(connection, 1),
    ]);

    await printParticipants(connection, [
      ['programId', program.programId],
      ['initializer', initializer.publicKey],
      ['playerA', playerA.publicKey],
      ['playerB', playerB.publicKey],
    ]);
  });

  async function makeMove(player: web3.Keypair, row: number, col: number): Promise<void> {
    console.log('Player', player.publicKey.toBase58(), 'makes a move in row', row, 'and column', col);
    const instruction = await program.methods
      .makeMove(lobbyName, row, col)
      .accounts({ player: player.publicKey, })
      .instruction();

    await sendAnchorInstructions(connection, [instruction], [player]);
  }

  it('The Game was initialized', async () => {
    console.log('The initializer initializes the game account');
    console.log('Lobby name:      ', lobbyName);
    console.log('Delay slots:     ', delaySlots);
    console.log('Required amount: ', requiredAmountInLamports / web3.LAMPORTS_PER_SOL, 'SOL');

    const instruction = await program.methods
      .initialize(
        lobbyName,
        new anchor.BN(requiredAmountInLamports),
        new anchor.BN(delaySlots),
      )
      .accounts({
        initializer: initializer.publicKey,
        playerA: playerA.publicKey,
        playerB: playerB.publicKey,
      })
      .instruction();

    await sendAnchorInstructions(connection, [instruction], [initializer]);
  });

  it('Player A made a move', async () => {
    await makeMove(playerA, 0, 0);
  });

  it('Player B made a move', async () => {
    await makeMove(playerB, 1, 0);
  });

  it('Player A made a move', async () => {
    await makeMove(playerA, 0, 1);
  });

  it('Player B made a move', async () => {
    await makeMove(playerB, 1, 1);
  });

  it('Player A made a move', async () => {
    await makeMove(playerA, 0, 2);
  });

});