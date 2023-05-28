use uuid::Uuid;

struct InventoryItem{
    name: String,
    quantity: u32,
}

struct Inventory{
    id: Uuid,
    item: Vec<InventoryItem>,
}

trait inventoryItem{

    fn createNewInventoryItem(name: String, qte: u32) -> InventoryItem;
    fn addNewItemInInventory(name: String, qte: u32, it: &mut Inventory);

}

impl inventoryItem for InventoryItem{
    fn createNewInventoryItem(name: String, qte: u32) -> InventoryItem {
        InventoryItem{
            name: name,
            quantity: qte
        }
    }

    fn addNewItemInInventory(name: String, qte: u32, it: &mut Inventory) {
        let v = <InventoryItem as inventoryItem>::createNewInventoryItem(name, qte);
        it.item.push(v);
    }

}

trait iventoryAction{
    fn printItensForInventory(&self);
}

impl iventoryAction for Inventory{

    fn printItensForInventory(&self) {

        print!("itens exists in Inventory by UUID is equals for = {}", self.id);
        println!();

        self.item.iter().for_each(|x| {
            println!("item {} quantity {}", x.name, x.quantity);
        })

    }

}

fn main() {

    let mut inv = &mut Inventory{
        id: Uuid::new_v4(),
        item: Vec::new()
    };

    <InventoryItem as inventoryItem>::addNewItemInInventory("macbook".to_string(),3, &mut inv);
    inv.printItensForInventory();

}
