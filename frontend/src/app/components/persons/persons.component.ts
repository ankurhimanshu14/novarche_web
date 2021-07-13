import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-persons',
  templateUrl: './persons.component.html',
  styleUrls: ['./persons.component.css']
})
export class PersonsComponent implements OnInit {
  p_li:any;
  p_lis=[];
  constructor(private http : HttpClient){
      
}

ngOnInit(): void {
  this.http.get('http://localhost:8080/api/v1/routes/persons')
  .subscribe(Response => {
    if(Response){
    console.log(Response)
    this.p_li=Response;
    this.p_lis=this.p_li.list;
    }
  });
}}
