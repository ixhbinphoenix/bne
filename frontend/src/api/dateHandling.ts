export function getMondayAndFridayDates(): { currentMonday: string, currentFriday: string} {
    const now = new Date();
    const currentDayOfWeek = now.getDay(); // 0 = Sunday, 1 = Monday, ..., 6 = Saturday
  
    // Calculate the date of the Monday of the current week
    const monday = new Date(now);
    monday.setDate(now.getDate() - currentDayOfWeek + 1);
  
    // Calculate the date of the Friday of the current week
    const friday = new Date(monday);
    friday.setDate(monday.getDate() + 4);
  
    // Format the dates as strings in the format YYYYMMDD
    const currentMonday = monday.toISOString().slice(0, 10).replace(/-/g, '');
    const currentFriday = friday.toISOString().slice(0, 10).replace(/-/g, '');
  
    // Return an object with the Monday and Friday dates
    return { currentMonday, currentFriday };
}
export function shiftForward(monday: string, friday: string): { currentMonday: string, currentFriday: string } {
    const mondayDate: Date = new Date(Number(monday.slice(0, 4)), Number(monday.slice(4, 6)) - 1, Number(monday.slice(6)));
    const fridayDate: Date = new Date(Number(friday.slice(0, 4)), Number(friday.slice(4, 6)) - 1, Number(friday.slice(6)));
  
    const nextMondayDate: Date = new Date(mondayDate.setDate(mondayDate.getDate() + 7));
    const nextFridayDate: Date = new Date(fridayDate.setDate(fridayDate.getDate() + 7));
  
    const currentMonday: string = `${nextMondayDate.getFullYear()}${(nextMondayDate.getMonth() + 1).toString().padStart(2, '0')}${nextMondayDate.getDate().toString().padStart(2, '0')}`;
    const currentFriday: string = `${nextFridayDate.getFullYear()}${(nextFridayDate.getMonth() + 1).toString().padStart(2, '0')}${nextFridayDate.getDate().toString().padStart(2, '0')}`;
  
    return { currentMonday, currentFriday};
  }
  
export function shiftBackward(monday: string, friday: string): { currentMonday: string, currentFriday: string } {
    const mondayDate: Date = new Date(Number(monday.slice(0, 4)), Number(monday.slice(4, 6)) - 1, Number(monday.slice(6)));
    const fridayDate: Date = new Date(Number(friday.slice(0, 4)), Number(friday.slice(4, 6)) - 1, Number(friday.slice(6)));
  
    const prevMondayDate: Date = new Date(mondayDate.setDate(mondayDate.getDate() - 7));
    const prevFridayDate: Date = new Date(fridayDate.setDate(fridayDate.getDate() - 7));
  
    const currentMonday: string = `${prevMondayDate.getFullYear()}${(prevMondayDate.getMonth() + 1).toString().padStart(2, '0')}${prevMondayDate.getDate().toString().padStart(2, '0')}`;
    const currentFriday: string = `${prevFridayDate.getFullYear()}${(prevFridayDate.getMonth() + 1).toString().padStart(2, '0')}${prevFridayDate.getDate().toString().padStart(2, '0')}`;
  
    console.log(currentMonday, currentFriday)
    return { currentMonday, currentFriday};
  }
  