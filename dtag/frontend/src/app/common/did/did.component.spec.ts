import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DIDComponent } from './did.component';

describe('DIDComponent', () => {
  let component: DIDComponent;
  let fixture: ComponentFixture<DIDComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DIDComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(DIDComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
