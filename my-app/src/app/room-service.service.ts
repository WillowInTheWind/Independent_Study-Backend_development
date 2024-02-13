import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class RoomService {
  getRooms() {
    return this.http.get<RoomInfo[]>('../assets/rooms.json');
  }
  constructor(private http: HttpClient) { }
}

export class RoomInfo {
  RoomNumber: number | undefined;
  RoomCapacity: number | undefined;
}