import { useEffect, useState } from "react";
import { TeacherCard } from "../../model";
import { Table } from "react-bootstrap";
import BorrowingWindow from "./borrowingWindow";

type ReadonlyCard = Omit<TeacherCard, "teacher"> & {
  owner: string;
  is_teacher: boolean;
};

function CardTable() {
  const [cards, setCards] = useState<ReadonlyCard[]>([]);
  const [card, setCard] = useState<ReadonlyCard | null>(null);

  useEffect(() => {
    getCards().then((cards) => setCards(cards));
  }, []);

  return (
    <>
      {card ? (
        <BorrowingWindow
          card={card.id}
          isTeacher={card.is_teacher}
          showInitial={true}
          owner={card.owner}
          closeWindow={() => setCard(null)}
        />
      ) : (
        ""
      )}
      <Table>
        <thead>
          <tr>
            <th className="p-3">Owner</th>
            <th className="p-3">Issue date</th>
            <th className="p-3">Is teacher</th>
          </tr>
        </thead>
        <tbody>
          {cards.map((card, i) => (
            <tr
              onClick={() => {
                setCard(card);
              }}
              key={i}
            >
              <td className="p-3">{card.owner}</td>
              <td className="p-3">{card.issue_date}</td>
              <td className="p-3">{card.is_teacher.toString()}</td>
            </tr>
          ))}
        </tbody>
      </Table>
    </>
  );
}

async function getCards(): Promise<ReadonlyCard[]> {
  const cardsRes = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/card-readonly`
  );
  const cards = (await cardsRes.json()) as ReadonlyCard[];

  return cards;
}

export default CardTable;
