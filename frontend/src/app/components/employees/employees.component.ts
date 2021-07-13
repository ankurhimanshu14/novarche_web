import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-employees',
  templateUrl: './employees.component.html',
  styleUrls: ['./employees.component.css']
})
export class EmployeesComponent implements OnInit {
  e_li:any;
  e_lis=[];
  constructor(private http : HttpClient){
      
}

ngOnInit(): void {
  this.http.get('http://localhost:8080/api/v1/routes/employees')
  .subscribe(Response => {
    if(Response){
    console.log(Response)
    this.e_li=Response;
    this.e_lis=this.e_li.list;
    }
  });
}}
